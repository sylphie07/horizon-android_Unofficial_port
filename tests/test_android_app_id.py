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


    def test_app_name_is_passed_only_when_requested(self):
        self.assertIsNone(build.gradle_app_name_args(None))
        self.assertEqual(build.gradle_app_name_args('Sylphra'), ['-PappName=Sylphra'])

    def test_branded_apk_names_do_not_change_upstream_defaults(self):
        self.assertEqual(build.apk_output_name(None, public=True), 'horizon-android-public.apk')
        self.assertEqual(build.apk_output_name('Sylphra', public=True), 'sylphra-android-public.apk')
        self.assertEqual(build.apk_output_name('Sylphra', release_candidate=True), 'sylphra-android-release.apk')
        self.assertEqual(build.apk_output_name('Sylphra'), 'sylphra-android-dev.apk')

    def test_manifest_label_is_a_gradle_placeholder(self):
        manifest = (Path(__file__).resolve().parents[1] / 'packaging/android/AndroidManifest.xml').read_text()
        gradle = (Path(__file__).resolve().parents[1] / 'packaging/android/build.gradle').read_text()
        self.assertIn('android:label="${appName}"', manifest)
        self.assertIn("manifestPlaceholders = [appName:", gradle)
