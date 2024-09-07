# Tools
- Bruno

# Run locally
Make server visible to your android device
[Download ADB bridge](https://developer.android.com/tools/releases/platform-tools)
```
./tools/adb.exe reverse tcp:8080 tcp:8080
```

# Database
#database

## Schema
### Entity
Represents an item. A nfc tag serial number can have a Entity assigned. 
An entity can have a owner entity, which means that it is assigned or located there.