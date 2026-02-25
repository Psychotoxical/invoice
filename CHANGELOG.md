# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.18] - 2026-02-25

### Added
- **Seller Contact Person**: Added first name and last name fields to the seller form and database. Now the contact person's name appears on the generated PDF invoices.

## [1.0.17] - 2026-02-25

### Changed
- **Rebranding**: Renamed the application from "Rechnung" to "VibeBill" for a more modern and international feel.
- **App Icon**: Trimmed unnecessary transparent borders from the application icon to make it appear larger and clearer in the taskbar and dock.

## [1.0.16] - 2026-02-25

### Fixed
- **PDF Layout**: Adjusted the table header padding and vertical centering for a cleaner look.

## [1.0.15] - 2026-02-24

### Added
- **Full-Featured Inline Modals**: Added the ability to completely fill out new customer and seller details (including logo uploads and banking information) directly from the invoice creation form without losing context.
- **Save to Catalog**: Manually entered invoice line-items can now be saved directly to the product catalog with one click.
- **Brand Colors**: Sellers can now configure a custom brand color (hex) which dynamically tints the "Modern" PDF invoice export template and the yearly overview document.
- **Dashboard Quick-Open**: Added double-click functionality to instantly open invoices from the "Recent Invoices" list on the dashboard.

### Fixed
- **PDF Layout**: Increased vertical separation between sender and recipient addresses on invoices.
- **PDF Word Wrapping**: Implemented a custom text-wrapping function for PDF generation to forcefully break extremely long words in item descriptions that exceeded column boundaries.
- **Dark Mode Visibility**: Fixed unreadable text colors for the "Top Customers" widget on the dashboard when using the dark theme.

## [1.0.14] - 2026-02-24

### Changed
- **Default Language**: Changed the default application language from German (`de`) to English (`en`) for new installations.

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
