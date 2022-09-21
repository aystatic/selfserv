<p align="center">
    <img src="https://raw.githubusercontent.com/aystatic/selfserv/master/assets/logo.svg" alt="selfserv">
</p>
<p align="center">
    self-hosted sideloading
</p>

## cli

`selfctl` can be used to interact with selfserv.

```
$ selfctl refresh
Resigning iSH.ipa... (7/7)
Signed 7 apps in 2:14.448

$ selfctl -u tim@apple.com iSH.ipa
Multiple devices connected:
    (1) Tim's iPhone
    (2) Tim's iPad Pro

:: Devices to sideload to (eg: 1 2 3, 1-3):
:: 1
```
