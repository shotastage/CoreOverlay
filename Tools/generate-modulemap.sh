#!/usr/bin/env bash

echo "module CWasmer [system] [extern_c] {" > $1/module.modulemap
echo "  header \"$HOME/.wasmer/include/wasmer.h\"" >> $1/module.modulemap
echo "  link   \"libwasmer.a\"" >> $1/module.modulemap
echo "  export *" >> $1/module.modulemap
echo "}" >> $1/module.modulemap
