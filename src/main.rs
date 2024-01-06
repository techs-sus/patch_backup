use clap::Parser;
use plist::{Data, Date, Dictionary, Value};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
struct InfoPlist {
	#[serde(rename = "Applications")]
	applications: Dictionary,
	#[serde(rename = "Build Version")]
	build_version: String,
	#[serde(rename = "Device Name")]
	device_name: String,
	#[serde(rename = "Display Name")]
	display_name: String,
	#[serde(rename = "GUID")]
	guid: String,
	/* IMEI */
	#[serde(rename = "IMEI")]
	imei: Option<String>,
	#[serde(rename = "Installed Applications")]
	installed_applications: Vec<Value>,
	#[serde(rename = "Last Backup Date")]
	last_backup_date: Date,
	#[serde(rename = "Product Name")]
	product_name: String,
	#[serde(rename = "Product Type")]
	product_type: String,
	#[serde(rename = "Product Version")]
	product_version: String,
	#[serde(rename = "Serial Number")]
	serial_number: String,
	#[serde(rename = "Target Identifier")]
	// UDID
	target_identifier: String,
	#[serde(rename = "Target Type")]
	target_type: String,
	#[serde(rename = "Unique Identifier")]
	// UDID
	unique_identifier: String,
	#[serde(rename = "iTunes Files")]
	itunes_files: Dictionary,
	#[serde(rename = "iTunes Settings")]
	itunes_settings: Dictionary,
	#[serde(rename = "iTunes Version")]
	itunes_version: String,
}

#[derive(Deserialize, Serialize)]
struct Lockdown {
	#[serde(rename = "com.apple.MobileDeviceCrashCopy")]
	mobile_device_crash_copy: Dictionary,
	#[serde(rename = "com.apple.TerminalFlashr")]
	terminal_flashr: Dictionary,
	#[serde(rename = "com.apple.mobile.data_sync")]
	mobile_data_sync: Dictionary,
	#[serde(rename = "com.apple.Accessibility")]
	accessibility: Dictionary,

	#[serde(rename = "ProductVersion")]
	product_version: String,
	#[serde(rename = "ProductType")]
	product_type: String,
	#[serde(rename = "BuildVersion")]
	build_version: String,

	#[serde(rename = "com.apple.mobile.iTunes.accessories")]
	itunes_accessories: Dictionary,
	#[serde(rename = "com.apple.mobile.wireless_lockdown")]
	mobile_wireless_lockdown: Dictionary,
	// UDID
	#[serde(rename = "UniqueDeviceID")]
	unique_device_id: String,
	#[serde(rename = "SerialNumber")]
	serial_number: String,
	#[serde(rename = "DeviceName")]
	device_name: String,
}

#[derive(Deserialize, Serialize)]
struct ManifestPlist {
	#[serde(rename = "BackupKeyBag")]
	backup_key_bag: Data,
	#[serde(rename = "Version")]
	version: String,
	#[serde(rename = "Date")]
	date: Date,
	#[serde(rename = "SystemDomainsVersion")]
	system_domains_version: String,
	#[serde(rename = "WasPasscodeSet")]
	was_passcode_set: bool,
	#[serde(rename = "Lockdown")]
	lockdown: Lockdown,
	#[serde(rename = "Applications")]
	applications: Dictionary,
	#[serde(rename = "IsEncrypted")]
	is_encrypted: bool,
}

/// Patches an idevicebackup2 backup.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
	/// The build version to be injected
	#[arg(long)]
	build_version: String,

	/// The product type to be injected
	#[arg(long)]
	product_type: String,

	/// The serial number to be injected
	#[arg(long)]
	serial_number: String,

	/// The udid to be injected
	#[arg(long)]
	udid: String,

	/// The imei to be injected
	#[arg(long)]
	imei: Option<String>,

	/// The backup directory for injection
	#[arg(index = 1)]
	backup_directory: PathBuf,
}

fn main() {
	// BuildVersion, ProductType, SerialNumber, UDID, BackupDirectory, Imei?
	let args = Args::parse();
	let build_version = args.build_version;
	let product_type = args.product_type;
	let serial_number = args.serial_number;
	let udid = args.udid;
	let backup_directory = args.backup_directory;
	let imei = args.imei;

	let info_file: PathBuf = backup_directory.join("Info.plist");
	let manifest_file: PathBuf = backup_directory.join("Manifest.plist");
	let mut info: InfoPlist = plist::from_file(&info_file).unwrap();
	let mut manifest: ManifestPlist = plist::from_file(&manifest_file).unwrap();

	/*
		info.plist
		- BuildVersion
		- IMEI?
		- Product Type
		- Serial Number
		- TargetId <- udid (Target Identifier)
		- UDID <- udid (Unique Identifier)
	*/
	info.build_version = build_version.to_owned();
	/* no imei backups available :( */
	info.imei = imei;
	info.product_type = product_type.to_owned();
	info.serial_number = serial_number.to_owned();
	info.target_identifier = udid.to_owned();
	info.unique_identifier = udid.to_owned();

	/*
	lockdown struct
		- BuildVersion
		- ProductType
		- SerialNumber
		- UDID (UniqueDeviceID)
	*/
	manifest.lockdown.build_version = build_version.to_owned();
	manifest.lockdown.product_type = product_type.to_owned();
	manifest.lockdown.serial_number = serial_number.to_owned();
	manifest.lockdown.unique_device_id = udid.to_owned();

	// write the backup files
	plist::to_file_xml(info_file, &info).unwrap();
	plist::to_file_binary(manifest_file, &manifest).unwrap();
}
