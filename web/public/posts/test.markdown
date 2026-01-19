Feeling defeated by my failure to solve any of the 'hard' crypto problems on display at HKCERT CTF, I turned my attention to Cook Book: it had the fewest lines of code. Therefore, it should be the easiest. Look at how few lines of code there are: 

```python
from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad
from Crypto.Random import get_random_bytes
BLOCK_SIZE = 16 # Bytes

key = get_random_bytes(BLOCK_SIZE)
cipher = AES.new(key, AES.MODE_ECB)
with open("flag.bmp", "rb") as fp:
    pt = fp.read()
ct = cipher.encrypt(pad(pt, BLOCK_SIZE))
with open("flag.enc","wb") as fp:
    fp.write(ct)
```

\
Finding the exploit should be easy!! Scanning through, the program seems very simple: it just uses the AES module from Crypto.Cipher, encrypts a file, then... saves it. Wait... where is the bug? It's just...... encrypting a file? Hm...

DuckDuckGoing the few details we have, I found that the `.bmp` extension is for image bitmaps, and `AES.MODE_ECB` represents the electronic ~~cook~~ code book AES format. [Looking through the specifics of ECB](https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#Electronic_codebook_(ECB)), we can find an interesting detail: unlike other AES formats, ECB has no 'psuedo-randomness': given the same plaintext and key, it will always return the exact same ciphertext:

![image](https://upload.wikimedia.org/wikipedia/commons/thumb/d/d6/ECB_encryption.svg/768px-ECB_encryption.svg.png)

While this may be acceptable for one block of data, when you have hundreds of blocks of data - like, if you were trying to encrypt an  image - you have a high chance of running into 'collisions', where you have multiple identical ciphertext blocks. We might be able to use these collisions to extract information from the cipher without even having to decode it! Look at this example:

![image](https://miro.medium.com/v2/resize:fit:1400/1*zw8Juc6NoZheJt4k61tuEQ.png)

If we can try to employ the same technique on the encrypted data provided, we might be able to make some progress. We will break the encrypted data back into its cipher blocks, then try to decipher some kind of pattern.

```python
blocks = [ct[i:i+16] for i in range(0, len(ct), 16)] # turns back into array of blocks of 16
```
\
We don't know what the file dimensions are. For now, we will assume that it is approximately a square. There are 196612 blocks, so let's give the image a size of 444. We will now assign each cipher with its own unique colour, and see what the output is. Maybe there'll be a flag!

```python
height = 444
width = 444

image = Image.new('RGB', (width, height), "black") // Pillow image library
pixels = image.load()

for i in range(len(blocks)):
x = i % width
y = i // width
block = blocks[i]
if dict[blocks[i]] == 1:
        pixels[x, y] = (255, 0, 0)
else:
    color = (hash(block) % 256, (hash(block) // 256) % 256, (hash(block) // 65536) % 256)
    pixels[x,y] = color
```
\
Which returns the following image:
    
![output](../imgs/image.png)

Uh... There definitely seems to be some data... It's just not entirely... comprehensible. Maybe the dimensions are the problem: it looks like if everything was lined up better, there might be something interesting. As we still can't find a definite resolution, what if we just brute-force it instead? We can just loop over every possible resolution, then go through them to see what happened.

```python
for width in range(100,1000, 10):
    height = len(blocks)//width + 1 
    // ... rest is the same

    image.save(f"output/{width}-{height}-output.png")
```
\
Now, we can go through each image one by one and... WOW!!!! LOOK AT THAT!!

![almost](../imgs/almostflag.png)

If we just mirror and reverse it, we get...

![flag](../imgs/flag.png)

```hkcert24{do_you_know_the_cool_penguin}```. herherha
