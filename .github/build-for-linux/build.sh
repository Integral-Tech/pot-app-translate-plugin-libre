#!/bin/bash

# SPDX-FileCopyrightText: 2024 Integral <integral@member.fsf.org>
#
# SPDX-License-Identifier: GPL-3.0-or-later

cargo build --target "$INPUT_TARGET" --release
mv target/$INPUT_TARGET/release/libplugin.so ./plugin.so
