PROJECT = yafetch
OBJECTS = src/script.o src/func.o src/main.o
LUACFLAGS := $(shell pkg-config --cflags lua5.4)
LUALDFLAGS := $(shell pkg-config --libs lua5.4)

CONF ?= /usr/share/yafetch
PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin

all: $(PROJECT)

$(PROJECT): $(OBJECTS)
	$(CC) $(CFLAGS) $(LUACFLAGS) $^ $(LUALDFLAGS) -o $@

config: $(PROJECT)
	mkdir -p $(DESTDIR)$(CONF)
	cp init.lua $(DESTDIR)$(CONF)/init.lua

install: $(PROJECT)
	mkdir -p $(DESTDIR)$(BINDIR)
	install -Dm755 yafetch $(DESTDIR)$(BINDIR)

clean:
	rm -f $(PROJECT) $(OBJECTS)
