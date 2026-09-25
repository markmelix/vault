## Vault
PDKDF2 and AES Rust implementations wrapped to Python FFI-binded library.

### Build

Install cargo make to build the library with convenience:
```bash
cargo install cargo-make
cargo make release  # this will place resulting vault.so at the repository root
```

### Usage

To provide Python bindings vault uses [rust-cpython](https://github.com/dgrunwald/rust-cpython).
The library was tested to work correctly with Python 3.10 - 3.11.

Setup environment having the corresponding Python version with uv:
```bash
uv venv --seed --python 3.12
source .venv/bin/activate
python
```

Then see the usage example and try it out:
```python
from vault import Vault
import vault as vlt

# bytes_to_bits function just converts specified bytes amount to the bits one.
# In this example we are converting 1024 bytes to the bits.
# There is also mb_to_bits functions.
# It's highly recommended to use powers of 2 as value for these functions
# E. g. 512, 1024, 2048 etc.
vault_size = vlt.bytes_to_bits(1024)

vault_password = 'my-difficult-vault-password123'

# Here we use bin extension because vault file contains only binary data.
vault_path = 'vault.bin'

# Create new Vault with provided password and private key size in bits.
# Note that now vault is just inside memory, so
# to save Vault to the file use save method.
vault = Vault(vault_password, vault_size)

# Or open existing Vault file.
vault = Vault.open(vault_path)

# Encrypt data to be stored in Vault.
# Note that data previously stored in Vault will be overwritten.
vault.encrypt(vault_password, 'My super important secrets: ...')

# Encrypt data to be stored in Vault not overwritting prevous data.
vault.encrypt_append(vault_password, '\nJust useless data.')

# Save Vault to the file.
# It's recommended to use this method after every encryption method.
vault.save(vault_path)

# Decrypt data stored in Vault.
# This will return something like that:
# 'My super important secrets: ...\nJust useless data.'
vault.decrypt(vault_password)
```
