ifeq ($(OS),Windows_NT)
    SHELL=CMD.EXE
    SET=set
    NUL=nul
    EXE=.exe
    RM=del
    VERSION:=v$(shell powershell "(cargo metadata --no-deps --format-version 1 | ConvertFrom-Json).packages.version")
else
    SET=export
    NUL=/dev/null
    EXE=
    RM=rm
    VERSION:=v$(shell gawk '/version/{ gsub(/[="]/,"",$$NF) ; print $$NF ; exit }' Cargo.toml)
endif

NAME:=$(subst -rs,,$(notdir $(CURDIR)))

all:
	cargo fmt
	cargo build --release

clean-dist:
	$(RM) $(NAME)-*.zip

TARGET=$(ARCH)-$(VENDOR)-$(SYS)-$(ABI)
_dist:
	cargo build --release --target $(TARGET)
	zip -j $(NAME)-$(VERSION)-$(SYS)-$(ARCH).zip target/$(TARGET)/release/$(NAME)$(EXE)

dist:
	$(MAKE) _dist ARCH=i686   VENDOR=pc SYS=windows ABI=msvc
	$(MAKE) _dist ARCH=x86_64 VENDOR=pc SYS=windows ABI=msvc

manifest:
	make-scoop-manifest *-windows-*.zip > $(NAME).json

release:
	gh release create -d --notes "" -t $(VERSION) $(VERSION) $(wildcard $(NAME)-$(VERSION)-*.zip)

.PHONY: dist manifest release
