# Google Forms Printer

A small program that:
- Polls Google Forms for new responses
- Generates a card -sized PDF with the respondent's name and company (pulled from the form)
- Sends that PDF to a printer

This uses the [`winprint` crate](https://crates.io/crates/winprint), which only works on Windows.

## First setup
1. Download (or build) this program \
   *If building from source, be sure to copy `pdfium.dll` from your `build` folder to the same directory as the executable.* \
   *I don't know why it isn't compiled in.*
2. Create OAuth credentials in the Google Developer Console
3. Copy the client secrets to `cfg/secrets.json`
4. Run the program
5. Go through OAuth authentication to connect to Google
6. Enter the form's ID. \
   If your edit URL is `https://docs.google.com/forms/d/1NB2N7t01fjg345098jh_f48vd29-Q/edit`, then the ID is `1NB2N7t01fjg345098jh_f48vd29-Q`.
7. Select which questions from the form pertain to the respondent's name and company
8. Select which printer to send document to
9. You should see `Good to go!`

## Customising the PDF
The PDF is generated with [Typst](https://typst.app/); edit `pdf/pdf.typ` to change the output PDFs.

## Rate Limiting
According to Google's API documentation, the rate limit for Google Forms is 180 GETs per minute, per project, per user.
This program polls once per second, so the rate limit shouldn't be an issue.
