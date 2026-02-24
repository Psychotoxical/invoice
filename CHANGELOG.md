# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.13] - 2026-02-24

### Added
- **About Box Details**: Extended the "About" dialog to include explicit Open Source status and a "vibecoded" credit.
- **Project Structure**: Updated `package.json` and Tauri build metadata for public Git availability. 

## [1.0.12] - 2026-02-24

### Added
- **Initial Public Release**: The repository is now fully Open Source under the MIT License.
- **Database Import/Export**: Added robust `.db` export and import functionality in the Settings menu for easy backups and migrations.
- **PDF Invoices**: Complete PDF generation workflow for invoices with automatic tax calculation.
- **Multilingual UI**: Full interface translations in German (`de`) and English (`en`).
- **Dashboard Analytics**: Overview of open/paid invoices, recent revenue, and top customers.
- **Customer & Product Catalogs**: Manage robust local databases for reusable invoice items and clients.
- **Cross-Platform Bundles**: Setup GitHub Actions workflow to automatically cross-compile `.deb`, `.rpm`, `.AppImage`, `.nsis`, `.msi`, and macOS builds on release tags.

### Fixed
- Fixed Tauri v2 ACL configuration (`fs:scope`) to properly allow database read/write access from the `$APPCONFIG` paths during backups.
