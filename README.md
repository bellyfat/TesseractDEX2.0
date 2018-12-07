# TesseractDEX2.0
Decentralized Exchange Service extending the Tesseract protocol (currently in progress)

Original Tesseract Paper: [Tesseract](https://eprint.iacr.org/2017/1153.pdf)

Background/Motivation: Cryptocurrency exchanges are services where traders can buy, sell or exchange cryptocurrencies for other digital or fiat currencies. Since automated programs are usually used for high frequency trading and arbitrage, traders are able to modify their trading positions within milliseconds to respond to the rapid price fluctuations. This requires exchange services to be responsive in real-time. Unfortunately, most existing real-time cryptocurrency exchanges rely on third parties, which puts traders' funds at risk of being stolen or hacked. The Tesseract protocol remedies this risk by using Intel SGX enclaves to ensure that traders’ funds can never be stolen. A trusted clock is used within this protocol to ensure that the attacker cannot feed the SGX enclave with self-mined blocks and execute an eclipse attack. However, the assumption of a trusted clock does not always hold. In this work, we propose an extension to Tesseract that thwarts the need for trusted clocks while maintaining the real-time feature and preserving security. This approach, which utilizes bidirectional off-chain channels together with timelocked multisignature contracts, seeks to achieve these goals without introducing additional attack vectors.  

Description of Files:

