PROJECT = yafetch
OBJECTS = src/script.o src/func.o src/main.o
CFLAGS := $(CFLAGS) $(shell pkg-config --cflags lua5.4)
LDFLAGS := $(LDFLAGS) $(shell pkg-config --libs lua5.4)
STRIPFLAGS ?= --strip-all --remove-section=.note --remove-section=.gnu.version --remove-section=.comment --strip-debug --strip-unneeded

CONF ?= /usr/share/yafetch
PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin

all: $(PROJECT)

$(PROJECT): $(OBJECTS)
	$(CC) $(CFLAGS) $^ $(LDFLAGS) -o $@

config: $(PROJECT)
	mkdir -p $(DESTDIR)$(CONF)
	cp init.lua $(DESTDIR)$(CONF)/init.lua

strip: $(PROJECT)
	strip $(PROJECT) $(STRIPFLAGS) -o $(PROJECT)

install: $(PROJECT)
	mkdir -p $(DESTDIR)$(BINDIR)
	install -Dm755 yafetch $(DESTDIR)$(BINDIR)

clean:
	rm -f $(PROJECT) $(OBJECTS)
