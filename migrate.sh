export DYLD_LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:$DYLD_LIBRARY_PATH"
export DYLD_LIBRARY_PATH="$(brew --prefix libpq)/lib:$(brew --prefix libiconv)/lib:$DYLD_LIBRARY_PATH"

diesel migration run
