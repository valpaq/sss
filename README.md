Hi, here you can see simple implementation of Shamir Secret Sharing.

#### Clone ####
 ```bash
git clone git@github.com:valpaq/sss.git && cd sss/
```

**To build:**
```bash
cargo build --release
```

**Run with split mode:**
```bash
./target/release/sss --command split --read-from input_split.txt
```
**Result will be like that:**
```bash
1  6a61677561722076616375756d206d6f73717569746f206163696420696d70726f7665206e6578742070656f706c65206567c4ef96a1e74e68e4b0d594ec1a665f43b3afab92b8cc6a3b908bb30695cb6c 
2  6a61677561722076616375756d206d6f73717569746f206163696420696d70726f7665206e6578742070656f706c65206670e714ed4b12157d087457c0eb44eb6e2173e27d9f44b25f00f53dbf77af07a7 
3  6a61677561722076616375756d206d6f73717569746f206163696420696d70726f7665206e6578742070656f706c65206780d9d37664e2ba5cd7abf8eb62a00294fead04969506175770957b93c8b52316 
4  6a61677561722076616375756d206d6f73717569746f206163696420696d70726f7665206e6578742070656f706c652068979d2b31ef593d085257b914522babd3db5f15f673fcfb538a71452ff9a81db9 
5  6a61677561722076616375756d206d6f73717569746f206163696420696d70726f7665206e6578742070656f706c652069b5311c1fea759d7f7877983bb9e7e72ab78a169d3c295e534e889a940a87f790 
6  6a61677561722076616375756d206d6f73717569746f206163696420696d70726f7665206e6578742070656f706c65206ad995a6405637dbc24a0b966199d4b499932e068aed8b4056bcdb7bbffb54b09b 
7  6a61677561722076616375756d206d6f73717569746f206163696420696d70726f7665206e6578742070656f706c65206c04cac993329ff7d0c713b385f1f214206e4ae5bf8822a15dd569e8b3cc0e48da 
8  6a61677561722076616375756d206d6f73717569746f206163696420696d70726f7665206e6578742070656f706c65206d36d086187fadf1aaef8fefa8c24005bf48e0b43b0bef81689833e16f7cb4c04d 
9  6a61677561722076616375756d206d6f73717569746f206163696420696d70726f7665206e6578742070656f706c65206e6fa6dbd03d61c950c3804aca0abe897622ef71fd78f1e077053965f30d4816f4 
10  6a61677561722076616375756d206d6f73717569746f206163696420696d70726f7665206e6578742070656f706c65206faf4dcaba6bbb7ec242e4c4e9cb6d9f44fc771f06cf29be891c7a763e7dc84ccf
```

**Run with recover mode:**
```bash
./target/release/sss --command recover --read-from input_recover.txt
```
**Result will be like that:**
```bash
jaguar vacuum mosquito acid improve next people describe large shell obey genuine
```
