# rust-vulnerable-apps

This repo contains examples of some common Rust related security vulnerabilities. These currently include:

*  CORS (Cross-Origin Resource Sharing) Misconfiguration: This occurs when improper CORS policies allow unauthorized domains to access resources, leading to potential data leaks or unauthorized actions.

*  Hardcoded Secret: Storing sensitive information such as API keys, passwords, or tokens directly in the source code, which can be exposed if the code is shared or compromised.

*  SQL Injection (SQLi): A vulnerability that allows an attacker to manipulate SQL queries by injecting malicious input, leading to unauthorized access, data leaks, or even database compromise.

*  Server-Side Request Forgery (SSRF): An attacker tricks the server into making requests to internal or external resources that the attacker wouldn’t normally have access to, which could lead to information disclosure or further attacks.

*  Cross-Site Scripting (XSS): A vulnerability that allows attackers to inject malicious scripts into web pages viewed by other users, potentially leading to data theft, session hijacking, or spreading malware.
