
APP=gba
TARGET=arm7tdmi
OUTDIR=target/$(TARGET)/debug

all: build size fix

build:
	xargo rustc -- -C link-arg=-emain

fix: build
	arm-none-eabi-objcopy -v -O binary $(OUTDIR)/$(APP) $(OUTDIR)/$(APP).gba
	gbafix $(OUTDIR)/$(APP).gba

size: build
	arm-none-eabi-size $(OUTDIR)/$(APP)

dump: build
	arm-none-eabi-objdump -CDS $(OUTDIR)/$(APP)

nm: build
	arm-none-eabi-nm -C $(OUTDIR)/$(APP)

debug:
	arm-none-eabi-gdb $(OUTDIR)/$(APP) --tui --eval-command="tar rem :55555"

clean:
	rm -rf $(OUTDIR)

# 