# course-proj
# TODO: 
1) Think about what is project suppose to do
2) Think how to store tests
3) Think how to add roles into token
4) Create UI for Unity project



# Usage
Must downlaod & unpack `gettext 0.21 and iconv 1.16 Shared` from [here](http://mlocati.github.io/articles/gettext-iconv-windows.html). Then rename `libintl-8.dll` to `libintl-9.dll` and paste it near .exe file.

``` 
1) cargo check
```
```
2) cargo run
```


# Routes
### `POST` /user
Creates user, require body with json in format:
```json
{
  "last_name": "Ethan",
  "first_name": "Black",
  "email": "ethBlack@gmail.com",
  "password": "MyPassIsVerySecret"
}
``` 
### `POST` AUTH
Authentificates user, require basicAuth with username and password. In this case username = email.
