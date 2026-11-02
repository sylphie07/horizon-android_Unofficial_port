import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location(
    'android_build',
    Path(__file__).resolve().parents[1] / 'tools/android_build.py',
)
build = importlib.util.module_from_spec(spec)
spec.loader.exec_module(build)


class AndroidApplicationIdTests(unittest.TestCase):
    def test_accepts_sylphra_release_and_debug_base_ids(self):
        self.assertEqual(build.validate_android_app_id('dev.sylphra.android'), 'dev.sylphra.android')
        self.assertEqual(build.validate_android_app_id('dev.sylphra.android.dev'), 'dev.sylphra.android.dev')

    def test_rejects_invalid_application_ids(self):
        for value in ('dev', '.sylphra.android', 'dev..android', 'dev.sylphra-android',
                      '1dev.sylphra', 'dev.sylphra.á'):
            with self.subTest(value=value):
                with self.assertRaises(build.argparse.ArgumentTypeError):
                    build.validate_android_app_id(value)

    def test_only_explicit_ids_override_the_upstream_default(self):
        self.assertIsNone(build.gradle_app_id_args(None))
        self.assertEqual(build.gradle_app_id_args('dev.sylphra.android'), ['-PappId=dev.sylphra.android'])

    def test_gradle_always_separates_debug_application_id(self):
        gradle = (Path(__file__).resolve().parents[1] / 'packaging/android/build.gradle').read_text()
        self.assertIn("applicationIdSuffix '.dev'", gradle)
        self.assertNotIn("if (!providers.gradleProperty('appId').isPresent())", gradle)
