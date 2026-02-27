# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.26] - 2026-02-27

### Added
- **Partial Payments Visibility**: Partial payments are now explicitly shown underneath the grand total in the Invoice and Dashboard lists.
- **Collapsible Navigation**: The application sidebar menu can now be toggled to a compact, icon-only mode to save space.
- **Shipping Costs**: Dedicated input fields for shipping costs and tax rates have been added to the invoice form, automatically calculated into the grand totals and appended to the PDF exports.
- **Mark as Paid Option**: Added a native checkbox when drafting invoices to immediately mark them as "Already Paid", which adjusts the resulting PDF payment instructions to "Payment already received".
- **Gross Price Input**: Added a "Brutto eingeben" (Gross Price) toggle for Product items (both in the catalog and quick-add form) to automatically reverse-calculate the net price based on the tax rate.
- **PDF Overwrite Warning**: The app will now explicitly prompt for confirmation if exporting an invoice PDF would overwrite an identically named file in the destination folder.

### Changed
- **Input Field UX**: Date pickers now automatically dismiss upon selection, and clicking into number fields (like price or quantity) auto-selects their entire contents for faster typing.
- **Invoice Status Logic**: Invoices marked as "Sent" (Versendet) are now logically evaluated as paid and no longer contribute to the "open payments" balance on the Dashboard.
- **UI Styling**: Removed borders and background colors from the top action bar to better integrate with the main content area, providing a cleaner, more modern interface.

## [1.0.25] - 2026-02-26

### Added
- **Help Center**: Added a comprehensive internal help guide (accessible via the `❓` icon) with detailed explanations for all core app functions.

### Changed
- **Navigation Flow**: Reordered the main sidebar navigation to improve workflow logic (Dashboard > Sellers > Products > Customers > Invoices > Settings).
- **Settings Location**: Moved the Settings menu from the top header into the main sidebar navigation.

## [1.0.24] - 2026-02-26

### Added
- **Unsaved Changes Confirmation**: Added a native dialog prompt to prevent accidental data loss when closing the Add/Edit modals for Customers, Sellers, and Products by clicking outside or pressing cancel.
- **Quick Seller Invoice Defaults**: Added the ability to specify invoice defaults (payment terms, tax rate, currency, standard note) directly when creating a new Seller from within the Invoice form.

## [1.0.23] - 2026-02-25

### Fixed
- **Fresh Install Crash**: Resolved a syntax error in the initial database creation script that caused fresh installations of version 1.0.22 to hang and fail to initialize the database completely.

## [1.0.22] - 2026-02-25

### Fixed
- **Fresh Install Database Error**: Fixed a critical issue where the app failed to initialize the database schema on fresh installations due to unsupported SQL migration statements, which prevented users from saving Sellers.

## [1.0.21] - 2026-02-25

### Added
- **Seller Invoice Defaults**: Invoices now support default payment terms, tax rates, currencies, and notes configurable *per seller*, rather than globally.
- **Improved PDF Logo Scaling**: The invoice logo no longer stretches incorrectly when exporting to PDF.
- **Detailed About Info**: Updated the settings page with a longer, more personal statement about the motivation behind the app and its "vibecoded" origins.

## [1.0.20] - 2026-02-25

### Fixed
- **Language Settings**: Fixed a long-standing issue where changing the UI language to German was not reliably persisting across restarts. Replaced the asynchronous SQLite query during app bootstrap with a robust, synchronous `localStorage` fallback to guarantee immediate translations when the app opens.

## [1.0.18] - 2026-02-25

### Added
- **Seller Contact Person**: Added first name and last name fields to the seller form and database. Now the contact person's name appears on the generated PDF invoices.

### Fixed
- **Database Migrations**: Fixed bug where the backend modified an outdated `rechnung.db` database file instead of `vibebill.db`, and resolved a SQLite syntax error regarding multiple `ALTER TABLE` statements.

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
