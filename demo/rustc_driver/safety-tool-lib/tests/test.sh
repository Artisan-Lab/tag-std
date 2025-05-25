cargo_expand() {
  local name=$1
  cargo expand --test "$name" >"tests/expanded/$name.rs"
  if [ $? -eq 0 ]; then
    echo "Expansion successful. Output saved to tests/expanded/$name.rs"
  else
    echo "Expansion failed."
  fi
}

cargo_expand "property_attrs"
