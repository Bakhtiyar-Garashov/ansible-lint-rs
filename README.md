# Ansible Lint for Rust

This is a Rust implementation of Ansible Lint, a tool for linting Ansible playbooks and roles. It aims to provide similar functionality to the original Ansible Lint tool.

## To-Do List

- [ ] Implement YAML parsing for playbooks and roles
- [ ] Create rule engine for detecting common issues and best practices
- [ ] Support for predefined profiles for rules
- [ ] Support detection of deprecated Ansible modules and parameters
- [ ] Implement custom rule configuration options
- [ ] Add support for excluding specific files or directories from linting
- [ ] Configurable via ansible lint config file
- [ ] Provide detailed documentation and usage examples

## Implemented linting rules

- [ ] args
- [ ] avoid-implicit
- [ ] complexity
- [ ] command-instead-of-module
- [ ] command-instead-of-shell
- [ ] deprecated-bare-vars
- [ ] deprecated-local-action
- [ ] deprecated-module
- [ ] empty-string-compare
- [ ] fqcn
- [ ] galaxy
- [ ] ignore-errors
- [ ] inline-env-var
- [ ] internal-error
- [ ] jinja
- [ ] key-order
- [ ] latest
- [ ] literal-compare
- [ ] load-failure
- [ ] loop-var-prefix
- [ ] meta-incorrect
- [ ] meta-no-tags
- [ ] meta-runtime
- [ ] meta-video-links
- [ ] name
- [ ] no-changed-when
- [ ] no-free-form
- [ ] no-handler
- [ ] no-jinja-when
- [ ] no-log-password
- [ ] no-prompting
- [ ] no-relative-paths
- [ ] no-same-owner
- [ ] no-tabs
- [ ] only-builtins
- [ ] package-latest
- [ ] parser-error
- [ ] partial-become
- [ ] playbook-extension
- [ ] risky-file-permissions
- [ ] risky-octal
- [ ] risky-shell-pipe
- [ ] role-name
- [ ] run-once
- [ ] sanity
- [ ] schema
- [ ] syntax-check
- [ ] var-naming
- [ ] warning
- [ ] yaml


The list items above are based on the features supported by the original Ansible Lint tool. For more information, please refer to the [Ansible Lint GitHub repository](https://github.com/ansible-community/ansible-lint).
