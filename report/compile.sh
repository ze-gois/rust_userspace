#!/bin/bash

# Script to compile LaTeX files for the userspace project report

set -e  # Exit on error

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print header
echo -e "${BLUE}====================================${NC}"
echo -e "${BLUE}  Userspace LaTeX Compiler Script  ${NC}"
echo -e "${BLUE}====================================${NC}"
echo

# Check if necessary LaTeX tools are installed
if ! command -v pdflatex &> /dev/null; then
    echo -e "${RED}Error: pdflatex is not installed.${NC}"
    echo -e "Please install TeX Live or MiKTeX:"
    echo -e "  Ubuntu/Debian: ${YELLOW}sudo apt-get install texlive-full${NC}"
    echo -e "  Fedora:        ${YELLOW}sudo dnf install texlive-scheme-full${NC}"
    echo -e "  macOS:         ${YELLOW}brew install --cask mactex${NC}"
    exit 1
fi

# Check arguments
if [ "$#" -eq 0 ]; then
    echo -e "${YELLOW}Usage: ./compile.sh [report|beamer|all]${NC}"
    echo -e "  ${YELLOW}report${NC}: Compile the complete report"
    echo -e "  ${YELLOW}beamer${NC}: Compile the beamer presentation"
    echo -e "  ${YELLOW}all${NC}:    Compile both documents"
    echo
    echo -e "${YELLOW}Example:${NC} ./compile.sh all"
    exit 1
fi

# Compile complete report
compile_report() {
    echo -e "${BLUE}Compiling complete report...${NC}"
    pdflatex userspace_complete_report.tex
    echo -e "${GREEN}First pass complete.${NC}"

    bibtex userspace_complete_report.aux 2>/dev/null || echo -e "${YELLOW}No bibliography to process.${NC}"
    echo -e "${GREEN}Bibliography processed.${NC}"

    pdflatex userspace_complete_report.tex
    echo -e "${GREEN}Second pass complete.${NC}"

    pdflatex userspace_complete_report.tex
    echo -e "${GREEN}Final pass complete.${NC}"

    echo -e "${GREEN}Complete report compiled successfully!${NC}"
}

# Compile beamer presentation
compile_beamer() {
    echo -e "${BLUE}Compiling beamer presentation...${NC}"
    pdflatex userspace_report.tex
    echo -e "${GREEN}First pass complete.${NC}"

    pdflatex userspace_report.tex
    echo -e "${GREEN}Final pass complete.${NC}"

    echo -e "${GREEN}Beamer presentation compiled successfully!${NC}"
}

# Clean auxiliary files
clean_aux() {
    echo -e "${BLUE}Cleaning auxiliary files...${NC}"
    rm -f *.aux *.log *.nav *.out *.snm *.toc *.bbl *.blg *.synctex.gz
    echo -e "${GREEN}Auxiliary files cleaned.${NC}"
}

# Process argument
case "$1" in
    report)
        compile_report
        ;;
    beamer)
        compile_beamer
        ;;
    all)
        compile_report
        echo
        compile_beamer
        ;;
    *)
        echo -e "${RED}Unknown option: $1${NC}"
        echo -e "${YELLOW}Usage: ./compile.sh [report|beamer|all]${NC}"
        exit 1
        ;;
esac

# Clean up auxiliary files
clean_aux

echo
echo -e "${GREEN}Done!${NC}"
if [ "$1" = "report" ] || [ "$1" = "all" ]; then
    echo -e "Report PDF: ${YELLOW}userspace_complete_report.pdf${NC}"
fi
if [ "$1" = "beamer" ] || [ "$1" = "all" ]; then
    echo -e "Presentation PDF: ${YELLOW}userspace_report.pdf${NC}"
fi

exit 0
