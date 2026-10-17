#ifndef TECTONIC_PDF_DOCUMENT_H
#define TECTONIC_PDF_DOCUMENT_H

#ifdef __cplusplus
extern "C" {
#endif

/* Called under the engine lock. End on the creating thread after native cleanup. */
void *tt_pdf_document_begin(void);
void tt_pdf_document_end(void *scope);

#ifdef __cplusplus
}
#endif
#endif
