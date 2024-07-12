#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint test_plugin.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
    s.name             = 'ldd_system_listen'
    s.version          = '0.0.1'
    s.summary          = 'A new Flutter FFI plugin project.'
    s.description      = <<-DESC
    A new Flutter FFI plugin project.
    DESC
    s.homepage         = 'http://example.com'
    s.license          = { :file => '../LICENSE' }
    s.author           = { 'Your Company' => 'email@example.com' }
    s.source           = { :path => '.' }
    s.source_files     = 'Classes/**/*'
    # s.dependency 'FlutterMacOS'
    # s.platform = :osx, '10.11'
    s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES' }
    s.swift_version = '5.0'
    s.script_phase = {
      :name => 'Build Rust library',
      :script => 'sh "$PODS_TARGET_SRCROOT/../cargokit/build_pod.sh" ../rust ldd_system_listen',
      :execution_position => :before_compile,
      :input_files => ['${BUILT_PRODUCTS_DIR}/cargokit_phony'],
      # Let XCode know that the static library referenced in -force_load below is
      # created by this build step.
      :output_files => ["${BUILT_PRODUCTS_DIR}/libldd_system_listen.a"],
    }
    s.pod_target_xcconfig = {
      'DEFINES_MODULE' => 'YES',
      'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386',
      'OTHER_LDFLAGS' => '-force_load ${BUILT_PRODUCTS_DIR}/libldd_system_listen.a',
    }
  end