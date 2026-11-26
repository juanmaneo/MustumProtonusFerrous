/// `RasEntryDlgW` from `dlls/rasdlg/rasdlg_main.c`. The C body only traces its arguments and
/// returns `FALSE`, so the port returns `false` for every input.
pub fn ras_entry_dlg_w() -> bool {
    false
}
