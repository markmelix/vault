# Vault
Vault Application written on Rust and Python.

# Application Structure
Rust part of the application is the backend. Python part is the frontend.
Rust part contains special Python bindings, so Python part can interact with Vault using Rust safety and speed.

# Python bindings explanations
Before you can use bindings in Python you need to compile `vault.so` library. Go to the rs directory of the repository root and run `cargo make release` command (note that before running this command you should install cargo-make program using `cargo install cargo-make` command). After that go to the py directory of the repository root and you may see `vault.so` file. Next, you can just import vault backend library as in example below.

See simple example which explains everything you need to know:
```python
from vault import Vault
import vault

# bytes_to_bits function just converts specified bytes amount to the bits one.
# In this example we are converting 1024 bytes to the bits.
# There is also mb_to_bits functions.
# It's highly recommended to use powers of 2 as value for these functions
# E. g. 512, 1024, 2048 etc.
vault_size = vault.bytes_to_bits(1024)

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

# Decrypt data stored in Vault.
# This will return something like that:
# 'My super important secrets: ...\nJust useless data.'
vault.decrypt(vault_password)

# Save Vault to the file.
# It's recommended to use this method after every encryption method.
vault.save(vault_path)
```
