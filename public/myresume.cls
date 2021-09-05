\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{myresume}[2019/07/04 My custom resume class]




\LoadClass{article}

\RequirePackage{titlesec}
\RequirePackage{graphicx}
\RequirePackage[margin=0.85in]{geometry}
\RequirePackage{float}
\RequirePackage[english]{babel}
\RequirePackage{url}
\RequirePackage{booktabs}
\RequirePackage{tabularx}
\RequirePackage{hyperref}

\hypersetup{
    colorlinks = true,
    linkcolor = blue,
    urlcolor = blue
}

\RequirePackage{fontawesome}
\RequirePackage{nopageno}
\RequirePackage{parskip}

\newcommand{\name}[1]{
    \center{\textbf{\Huge{#1}}}
}

\newcommand{\details}[3]{
    \center{\large{\faPhone{} \href{tel:#1}{#1} \faEnvelope{} \href{emailto:#2}{#2} \faGlobe{} \href{https://#3}{#3}}}
}

\newcommand{\sectionpart}[1]{
    \Large{\scshape{#1}} \titlerule \\ \normalsize \raggedright
}

\newcommand{\skill}[2]{
    \textbf{#1:} #2 \\
}

\newcommand{\experience}[3]{
    \begin{tabularx}{\textwidth}{ >{\hsize=0.25\hsize}X  >{\hsize=0.5\hsize}X >{\hsize=0.25\hsize}X }
        \textit{#1} & \textbf{#2} & #3 \\
    \end{tabularx}
}

