# Windows Installer Custom Actions for Real Version Information

[![releases](https://img.shields.io/github/v/release/heaths/msiverca.svg?logo=github)](https://github.com/heaths/msiverca/releases/latest)
[![ci](https://github.com/heaths/msiverca/actions/workflows/ci.yml/badge.svg?event=push)](https://github.com/heaths/msiverca/actions/workflows/ci.yml)

Windows Installer properties like `VersionNT` cannot be used for Windows versions newer than 8.1 because the APIs on which they rely lie about the version.
In fact, if you look in any recent Windows Installer verbose log, you'll see that AppCompat is also changing version information:

```text
MSI (c) (8C:20) [10:29:03:529]: APPCOMPAT: Uninstall VersionNT override found.
...
Property(C): VersionNT = 603
```

With even `VerfiyVersionInfo` deprecated in Windows 10, a custom action is needed to call into the kernel to get the correct version information.
This will also return separate properties for better, more intuitive version comparisons than what `VersionNT` or `VersionNT64` will support.

## Properties

The examples below are based on Windows 11 22H2.

Name | Description | Example
---- | ----------- | -------
VER_WINDOWS_MAJOR | The major version of Windows | 10
VER_WINDOWS_MINOR | The minor version of Windows | 0
VER_WINDOWS_BUILD | The build version of Windows | 22621

You can reference these in conditions for immediate custom actions, including those to schedule deferred custom actions
since the properties are declared as secure custom properties and passed from client to service.

## License

Licensed under the [MIT](LICENSE.txt) license.
