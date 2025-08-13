# Homework 1

## Team

- William Widjaja

## Grader Credentials
```
user: grader
pass: cSEe-1355?grad3r
```

## Link To My Site
<https://cse135-wwidjaja.online/>

## GitHub Auto Deploy
<https://youtu.be/42t3M-6ESvE>

## Site Login Credentials
```
user: grader
pw: !cse135EASY
```

## After Compression Changes
Transferred bytes is less than the total bytes of the original resources as I get the value of `gzip` in the Content-Encoding response header.

## Summary of Removing 'Server' Header
I used the `security2` plugin provided by the ModSecurity module to remove the 'Server' header from the response. This is done by adding a rule in the Apache configuration file.

initial-index.jpg - default Apache2 page to prove Apache is working
![initial-index](https://github.com/user-attachments/assets/fb8a7f40-ee49-4cd7-aab2-549512058ac4)

modified-index.jpg - first change to index.html
![modified-index](https://github.com/user-attachments/assets/7168e9a4-d5ab-43fc-8514-598e8efc8027)

validator-initial.jpg - validating your copied index.html
![validator-initial](https://github.com/user-attachments/assets/800652d4-49fc-45e0-a3a2-678af6422ab7)

vhosts-verify.jpg - demonstrating a functional domain.site, collector.domain.site, reporting.domain.sites
![vhosts-verify](https://github.com/user-attachments/assets/3cc1980e-604e-411d-b50c-115ade1e5330)

ssl-verify.jpg - verify your site uses HTTPS
![ssl-verify](https://github.com/user-attachments/assets/a18ffb4b-f25c-47f4-8f6b-03d34364a061)

github-deploy.mp4 - showing Github deploy process
![github-deploy](https://github.com/user-attachments/assets/92010ac6-8f3b-4bfd-bdfa-74d47e339a8d)

php-verification.jpg - demonstration of working php page
![php-verification](https://github.com/user-attachments/assets/0cb5af1c-9619-423e-b553-df9e07725d37)

compress-verify.jpg - demonstration of compression
![compression-verify](https://github.com/user-attachments/assets/28626280-fc14-4720-9661-3e0d8e354684)

header-verify.jpg - demonstration of 'server: cse135 server' response header
![header-verify](https://github.com/user-attachments/assets/edcaf02a-b3ae-4fb2-a886-46be64d54e9c)

error-page.jpg - demonstration of functional 404 page
![error-page](https://github.com/user-attachments/assets/660ab4d9-dbcc-4f11-aec7-b0391e5a47a7)

log-verification.jpg - showing you know where your log files are
![log-verification](https://github.com/user-attachments/assets/99d6ab00-3867-4478-bd39-107616571971)

report-verification.jpg - GoAccess screen capture
![report-verification](https://github.com/user-attachments/assets/b541462f-6b1c-4dfe-ad8a-1fd387358029)
