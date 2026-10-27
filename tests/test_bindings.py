import gc
import struct
import unittest
import weakref
from pathlib import Path

import shr_parser


class SweepDataTests(unittest.TestCase):
    def test_cached_sweeps_do_not_keep_parser_subclass_alive(self):
        class CachedParser(shr_parser.SHRParser):
            pass

        fixture = (
            Path(__file__).resolve().parents[2]
            / "shr_parser"
            / "tests"
            / "example_file"
            / "sweep.shr"
        )
        parser = CachedParser(str(fixture), shr_parser.SHRParsingType.PEAK)
        parser.sweeps = parser.get_sweeps()
        self.assertGreater(len(parser.sweeps), 0)
        parser_ref = weakref.ref(parser)

        del parser
        gc.collect()

        self.assertIsNone(parser_ref())

    def test_get_data_preserves_every_bin_and_parser_lifetime(self):
        fixture = (
            Path(__file__).resolve().parents[2]
            / "shr_parser"
            / "tests"
            / "example_file"
            / "sweep.shr"
        )
        contents = fixture.read_bytes()
        data_offset, sweep_count, sweep_length, first_bin_hz, bin_size_hz = (
            struct.unpack_from("<QII2d", contents, 8)
        )
        self.assertGreater(sweep_count, 1)
        self.assertGreater(sweep_length, 1)

        parser = shr_parser.SHRParser(str(fixture), shr_parser.SHRParsingType.PEAK)
        sweeps = parser.get_sweeps()
        self.assertEqual(len(sweeps), sweep_count)
        sweep_index = sweep_count // 2
        sweep = sweeps[sweep_index]
        self.assertEqual(sweep.sweep_number, sweep_index)

        # The sweep must retain its owner even after every other reference is gone.
        del parser, sweeps
        gc.collect()
        data = sweep.get_data()
        self.assertIsInstance(data, list)
        self.assertEqual(len(data), sweep_length)

        amplitudes_offset = data_offset + sweep_index * (48 + sweep_length * 4) + 48
        for bin_index, point in enumerate(data):
            with self.subTest(bin=bin_index):
                self.assertIsInstance(point, tuple)
                self.assertEqual(len(point), 2)
                frequency_hz, amplitude = point
                self.assertIsInstance(frequency_hz, float)
                self.assertIsInstance(amplitude, float)
                self.assertEqual(frequency_hz, first_bin_hz + bin_index * bin_size_hz)
                offset = amplitudes_offset + bin_index * 4
                expected_amplitude = struct.unpack_from("<f", contents, offset)[0]
                self.assertEqual(amplitude, expected_amplitude)
                self.assertEqual(
                    struct.pack("<f", amplitude), contents[offset : offset + 4]
                )


if __name__ == "__main__":
    unittest.main()
