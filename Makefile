# Copyright 2014 The Num-RS Developers. For a full listing of the authors,
# refer to the AUTHORS file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

RUSTC               = rustc
RUSTDOC             = rustdoc

SRC_DIR             = src
LIB_FILE            = $(SRC_DIR)/math/lib.rs

CRATE_NAME          = $(shell $(RUSTC) --crate-name $(LIB_FILE))
CRATE_FILES         = $(shell $(RUSTC) --crate-file-name $(LIB_FILE))

BUILD_PREFIX        = .
DOC_BUILD_DIR       = $(BUILD_PREFIX)/doc
LIB_BUILD_DIR       = $(BUILD_PREFIX)/lib
TEST_BUILD_DIR      = $(BUILD_PREFIX)/test

INSTALL_PREFIX      = $(HOME)/.rust
LIB_INSTALL_DIR     = $(INSTALL_PREFIX)/lib

all: lib doc

lib:
	mkdir -p $(LIB_BUILD_DIR)
	$(RUSTC) --out-dir=$(LIB_BUILD_DIR) -O $(LIB_FILE)

check:
	mkdir -p $(TEST_BUILD_DIR)
	$(RUSTC) --out-dir=$(TEST_BUILD_DIR) --test $(LIB_FILE)
	$(TEST_BUILD_DIR)/$(CRATE_NAME)

doc:
	mkdir -p $(DOC_BUILD_DIR)
	$(RUSTDOC) -o $(DOC_BUILD_DIR) $(LIB_FILE)

install: lib
	@mkdir -p $(INSTALL_PREFIX)
	@mkdir -p $(LIB_INSTALL_DIR)
	@ $(foreach crate, $(CRATE_FILES), \
		cp $(LIB_BUILD_DIR)/$(crate) $(LIB_INSTALL_DIR)/$(crate) && \
		echo "Installed $(crate) to $(LIB_INSTALL_DIR)" ; \
	)

uninstall:
	@-rm $(LIB_INSTALL_DIR)/lib$(CRATE_NAME)-*.rlib ||:

clean:
	rm -rf $(LIB_BUILD_DIR)
	rm -rf $(TEST_BUILD_DIR)
	rm -rf $(DOC_BUILD_DIR)

.PHONY: \
	all \
	lib \
	check \
	doc \
	install \
	uninstall \
	clean
