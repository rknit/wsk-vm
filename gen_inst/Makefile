all: clean copy_old run

clean:
	@echo "Cleaning up..."
	@rm -rf ./output

copy_old:
	@echo "Copying old files..."
	@rm -rf ./insts_old
	@cp -r -u ../src/insts ./insts_old 2> /dev/null || :

run:
	@echo "Running generator..."
	@rm -rf ../src/insts
	@./gen_inst.py
	@mv ./output ../src/insts
