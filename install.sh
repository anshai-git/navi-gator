#!/bin/sh

check_superuser() {
  if [ "$(id -u)" -ne 0 ]; then
    echo "Error: This script must be run with superuser privileges." >&2
    exit 1
  fi
}

check_existing_binary() {
  local bin_name="$1"
  local install_dir="$2"

  if [ -e "$install_dir/$bin_name" ]; then
    echo "Error: $bin_name is already installed in $install_dir." >&2
    exit 1
  fi
}

install_binary() {
  local bin_name="$1"
  local install_dir="$2"

  cp "$bin_name" "$install_dir"
}

set_permissions() {
  local bin_name="$1"
  local install_dir="$2"

  chmod +x "$install_dir/$bin_name"
}

verify_installation() {
  local bin_name="$1"
  local install_dir="$2"

  if [ -x "$(command -v $bin_name)" ]; then
    echo "Installation successful. $bin_name is now availbale from: '$install_dir'."
  else
    echo "Error: Installation failed." >&2
    exit 1
  fi
}

install_shell_wrapper() {
  local bin_name="navi-gator-sh"
  local install_dir="/usr/local/bin"

  check_existing_binary "$bin_name" "$install_dir"

  install_binary "$bin_name" "$install_dir"
  set_permissions "$bin_name" "$install_dir"
  verify_installation "$bin_name" "$install_dir"
}

install_rust_app() {
  local bin_name="navi-gator"
  local install_dir="/usr/local/bin"

  check_existing_binary "$bin_name" "$install_dir"

  install_binary "$bin_name" "$install_dir"
  set_permissions "$bin_name" "$install_dir"
  verify_installation "$bin_name" "$install_dir"
}

print_next_step() {
  echo ""
  echo "Add the following function to .bashrc:"
  echo ""
  echo "nav() {"
  echo "  . navi-gator-sh"
  echo "}"
  echo ""
}

check_superuser

install_shell_wrapper
install_rust_app
print_next_step
