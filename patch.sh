python - << 'EOF'
import toml
data = toml.load("Cargo.toml")
data['dependencies']['flate2']="1.0"
data['dependencies']['rxml']="0.8.1"
with open("Cargo.toml",'w') as f:
  toml.dump(data, f)
EOF
