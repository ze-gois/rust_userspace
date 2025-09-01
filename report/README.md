# Userspace Project Report

This directory contains LaTeX files for the Userspace project report for the Congresso de Iniciação Científica e Tecnológica (CICT) at UFRN.

## Files

- `userspace_report.tex`: Beamer presentation for oral presentation
- `userspace_complete_report.tex`: Complete work report following CICT format requirements

## Compilation Instructions

### Prerequisites

You'll need a LaTeX distribution installed on your system. The recommended options are:

- **Linux**: TexLive (`sudo apt-get install texlive-full` for Ubuntu/Debian)
- **Windows**: MiKTeX or TexLive
- **macOS**: MacTeX

### Compiling the Documents

#### Complete Report

To compile the complete report:

```bash
pdflatex userspace_complete_report.tex
bibtex userspace_complete_report.aux
pdflatex userspace_complete_report.tex
pdflatex userspace_complete_report.tex
```

The multiple runs are necessary to resolve references and citations properly.

#### Beamer Presentation

To compile the Beamer presentation:

```bash
pdflatex userspace_report.tex
pdflatex userspace_report.tex
```

### Output Files

The compilation will generate PDF files in the same directory:
- `userspace_complete_report.pdf`: The complete work report for submission
- `userspace_report.pdf`: The Beamer presentation for oral presentation

## Customization

Before submitting, make sure to:

1. Replace `[Nome do Orientador]` with your actual advisor's name
2. Verify that all information is accurate according to your research
3. Update the references section if necessary
4. Check if formatting meets CICT requirements

## Important CICT Requirements

According to the CICT guidelines, the complete work report should include:

- Title (max 200 characters)
- Abstract (max 1500 characters)
- Keywords (max 70 characters)
- Introduction (max 4000 characters)
- Method (max 6000 characters)
- Results and Discussion (max 20000 characters)
- Conclusions (max 3000 characters)
- References (max 8000 characters)

The LaTeX template is already structured to follow these requirements.

## ODS Information

Remember to verify which UN Sustainable Development Goals (ODS) your work relates to. This information should be included in your submission.

## Contact

If you have any questions about these files, contact:
- José Gois (ze.gois.00@gmail.com)