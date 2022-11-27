# Lectures

This folder contains lecture presentations and materials.

You can build lecture presentations from source by the following command (You'll need to set up `minted` package for Latex):

```bash
$ cd lecture-XX
$ xelatex -shell-escape -synctex=1 -interaction=nonstopmode slides.tex
```

Or open an already compiled PDF called `slides.pdf`.
