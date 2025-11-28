# MothrBox CLI Enhancement - Installation Instructions

## 📦 What You're Installing

This package adds complete CLI support for **ChaCha20-Poly1305** and **ECC** encryption to your MothrBox project, in addition to the existing AES-256-GCM support.

## 📋 Files Included

1. **mothrbox** - Enhanced CLI script (main executable)
2. **CLI_USAGE_GUIDE.md** - Complete usage documentation
3. **QUICK_REFERENCE.txt** - Quick command reference
4. **UPDATED_README.md** - Enhanced README content
5. **ENHANCEMENT_SUMMARY.md** - What changed and why
6. **test_all_methods.sh** - Test script for all encryption methods
7. **INSTALL.md** - This file

## 🚀 Installation Steps

### Step 1: Backup Your Current Script

```bash
cd ~/mothrbox_v2  # Or wherever your project is

# Backup existing script
cp mothrbox mothrbox.backup
```

### Step 2: Replace the CLI Script

```bash
# Copy the new enhanced script
cp /path/to/downloaded/mothrbox ./mothrbox

# Make it executable
chmod +x mothrbox
```

### Step 3: Add Documentation

```bash
# Copy documentation files to your project root
cp CLI_USAGE_GUIDE.md ./
cp QUICK_REFERENCE.txt ./
cp test_all_methods.sh ./

# Make test script executable
chmod +x test_all_methods.sh
```

### Step 4: Update Your README

Option A - Replace entire README:
```bash
cp UPDATED_README.md ./README.md
```

Option B - Manually merge (recommended):
1. Open `UPDATED_README.md`
2. Copy the sections you want
3. Paste into your existing `README.md`
4. Keep your existing project information

Recommended sections to add:
- Complete CLI Commands
- Encryption Methods (all three)
- Algorithm Comparison table
- Real-World Examples

### Step 5: Test Everything

```bash
# Make sure MothrBox is running
./mothrbox start

# Run the test suite
./test_all_methods.sh
```

Expected output:
```
╔════════════════════════════════════════════════════════╗
║         MOTHRBOX COMPLETE TEST SUITE                  ║
╚════════════════════════════════════════════════════════╝

✅ Container is running

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
TEST 1: AES-256-GCM
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ AES encryption successful!
✅ AES decrypt successful - data matches!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
TEST 2: ChaCha20-Poly1305
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ ChaCha20 encryption successful!
✅ ChaCha20 decrypt successful - data matches!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
TEST 3: ECC (Elliptic Curve Cryptography)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Key pair generated successfully
✅ ECC encryption successful!
✅ ECC decrypt successful - data matches!

╔════════════════════════════════════════════════════════╗
║              ALL TESTS PASSED SUCCESSFULLY!           ║
╚════════════════════════════════════════════════════════╝

✅ AES-256-GCM: Working ✓
✅ ChaCha20-Poly1305: Working ✓
✅ ECC (P-256): Working ✓
```

### Step 6: Commit Changes

```bash
git add mothrbox CLI_USAGE_GUIDE.md QUICK_REFERENCE.txt README.md test_all_methods.sh
git commit -m "Add ChaCha20 and ECC CLI commands with comprehensive documentation

- Added chacha-encrypt/chacha-decrypt commands
- Added ecc-encrypt/ecc-decrypt commands  
- Added keygen command for ECC keys
- Comprehensive documentation and examples
- Test suite for all encryption methods"

git push origin main
```

---

## ✅ Verification Checklist

After installation, verify:

- [ ] `./mothrbox help` shows all new commands
- [ ] `./mothrbox status` works
- [ ] `./test_all_methods.sh` passes all tests
- [ ] Documentation files are in place
- [ ] README is updated

---

## 🎯 Quick Start After Installation

### Try AES (Default)
```bash
./mothrbox start
echo "Test AES" > test.txt
./mothrbox encrypt test.txt "MyPass123"
# Save blob ID, then:
./mothrbox decrypt <blob-id> recovered.txt "MyPass123"
```

### Try ChaCha20 (Mobile-Optimized)
```bash
echo "Test ChaCha20" > test.txt
./mothrbox chacha-encrypt test.txt "MyPass123"
# Save blob ID, then:
./mothrbox chacha-decrypt <blob-id> recovered.txt "MyPass123"
```

### Try ECC (Public Key)
```bash
./mothrbox keygen  # Generates keys
echo "Test ECC" > test.txt
./mothrbox ecc-encrypt test.txt data/public.key
# Save blob ID, then:
./mothrbox ecc-decrypt <blob-id> recovered.txt data/private.key
```

---

## 🆘 Troubleshooting

### Issue: Permission Denied
```bash
chmod +x mothrbox
chmod +x test_all_methods.sh
```

### Issue: Container Not Running
```bash
./mothrbox start
./mothrbox status
```

### Issue: Commands Not Found
Make sure you replaced the script correctly:
```bash
ls -lh mothrbox
# Should show: -rwxr-xr-x ... mothrbox

head -n 5 mothrbox
# Should show: #!/bin/bash
#             # MothrBox CLI Wrapper
```

### Issue: Test Script Fails
1. Check container is running: `./mothrbox status`
2. Check .env configuration: `cat mothrbox_ts/.env`
3. Verify SUI_SECRET_KEY is set
4. Get testnet SUI: https://faucet.testnet.sui.io/

### Issue: Old Commands Still Work, New Don't
You might not have replaced the script:
```bash
./mothrbox chacha-encrypt --help
# Should show usage, not "unknown command"
```

---

## 📚 Next Steps

1. **Read the documentation:**
   - `CLI_USAGE_GUIDE.md` for detailed examples
   - `QUICK_REFERENCE.txt` for command reference

2. **Try the examples:**
   - See "Real-World Examples" in the usage guide
   - Experiment with different encryption methods

3. **Update your demo:**
   - Show all three encryption methods
   - Explain when to use each one

4. **Share your work:**
   - Update your GitHub README
   - Tweet about the enhancement
   - Share with hackathon judges

---

## 🎉 You're Done!

Your MothrBox now supports:
- ✅ AES-256-GCM (hardware-accelerated)
- ✅ ChaCha20-Poly1305 (mobile-optimized)
- ✅ ECC (public key cryptography)

All with simple, consistent CLI commands!

**Questions or issues?** Check:
- `ENHANCEMENT_SUMMARY.md` - What changed
- `CLI_USAGE_GUIDE.md` - How to use
- `QUICK_REFERENCE.txt` - Quick lookup

---

## 📞 Support

If you encounter any issues:
1. Check the troubleshooting section above
2. Review `ENHANCEMENT_SUMMARY.md`
3. Check your GitHub repo issues
4. Review Docker logs: `./mothrbox logs`

---

**Built for secure, unified, censorship-resistant storage** 🦋🔒