import { save } from '@tauri-apps/plugin-dialog';
import { create } from '@tauri-apps/plugin-fs';
import type { Invoice } from '../services/database';

export async function exportInvoicesCsv(invoices: Invoice[], locale: string): Promise<boolean> {
    const sep = ';';
    const isDE = locale === 'de';

    const headers = isDE
        ? ['Rechnungsnr.', 'Kunde', 'Verkäufer', 'Datum', 'Fällig', 'Status', 'Netto', 'MwSt', 'Brutto', 'Notizen']
        : ['Invoice No.', 'Customer', 'Seller', 'Date', 'Due Date', 'Status', 'Net', 'VAT', 'Gross', 'Notes'];

    const statusLabels: Record<string, Record<string, string>> = {
        de: { draft: 'Entwurf', sent: 'Versendet', paid: 'Bezahlt', overdue: 'Überfällig', cancelled: 'Storniert' },
        en: { draft: 'Draft', sent: 'Sent', paid: 'Paid', overdue: 'Overdue', cancelled: 'Cancelled' },
    };

    const rows = invoices.map(inv => [
        inv.invoice_number,
        inv.customer_name || '',
        inv.seller_name || '',
        inv.date,
        inv.due_date || '',
        statusLabels[locale]?.[inv.status] || inv.status,
        formatNum(inv.total_net, isDE),
        formatNum(inv.total_tax, isDE),
        formatNum(inv.total_gross, isDE),
        (inv.notes || '').replace(/[\r\n]+/g, ' '),
    ]);

    const csv = [
        headers.join(sep),
        ...rows.map(r => r.map(escapeCsv).join(sep)),
    ].join('\r\n');

    // BOM for Excel to recognize UTF-8
    const bom = '\ufeff';
    const content = bom + csv;

    const now = new Date();
    const ts = `${now.getFullYear()}${String(now.getMonth() + 1).padStart(2, '0')}${String(now.getDate()).padStart(2, '0')}`;
    const defaultName = `Rechnungen_${ts}.csv`;

    const filePath = await save({
        defaultPath: defaultName,
        filters: [{ name: 'CSV', extensions: ['csv'] }],
    });

    if (!filePath) return false;

    const file = await create(filePath);
    await file.write(new TextEncoder().encode(content));
    await file.close();
    return true;
}

function escapeCsv(val: string): string {
    if (val.includes(';') || val.includes('"') || val.includes('\n')) {
        return '"' + val.replace(/"/g, '""') + '"';
    }
    return val;
}

function formatNum(val: number, isDE: boolean): string {
    if (isDE) return val.toFixed(2).replace('.', ',');
    return val.toFixed(2);
}
