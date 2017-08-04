# adi_storage
Aldaron's Device Interface - Storage (adi_storage) is a Rust library for interfacing with a persistent storage device (ie: hard drive, solid state drive, sd card, flash drive, etc.).

## Storage System
adi_storage uses a cross-platform virtual filesystem.  The virtual filesystem's root directory is defined as followed:

* Aldaron's OS: `"/at_root/"`, Linux: `"/usr/local/share/at_root/"`, Windows: `"C:/Program Files/at_root/"`

Under the root directory, the storage devices are mounted.

* `"/~/"` Storage Device that was used for booting.
* `"/STORAGE DEVICE/"` An External Storage Device.

Under the storage device directory, we have the apps:

* `"/~/app@developer/"` An application's folder.  An application can only access files within their folder.  "Their folder" includes folders with the same name on other Storage Devices.

Under the app folder, we have the users:

* `"/~/app@developer/username"` A user's folder, contains all of their save files.

Under each of the users for a specific app, we have the internal files:

* `"/~/app@developer/username/internal/"` Internal App Data For The Specific User
