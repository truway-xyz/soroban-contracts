# Security policy

## Supported versions

The project is pre-release: only `main` is supported, and no contract is deployed from this
repository yet. Deployed instances, once they exist, are listed in
[`deployments/`](deployments/).

## Reporting a vulnerability

Please do not open a public issue.

- Preferred: GitHub's private vulnerability reporting, from the **Security** tab of this
  repository (**Report a vulnerability**).
- Alternative: email info@truway.xyz with the subject "Security report".

Include the affected contract and function, steps or a failing test that reproduces the issue,
and the impact you expect.

We aim to acknowledge reports within five business days and to share a remediation plan once
the issue is confirmed. We will credit reporters who wish to be named.

## Scope

In scope: the contracts in `contracts/`, the shared crate in `packages/`, and the build and
deployment scripts in this repository.

Out of scope here: the Truway web platform (report it to the same email address) and
third-party dependencies (report upstream).

## Past reviews

See [`audits/`](audits/).
