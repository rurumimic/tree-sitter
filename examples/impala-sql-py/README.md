# Impala SQL for Python

## Development Environment

### Pyenv & Virtualenv

```bash
pyenv install 3.11
pyenv virtualenv 3.11 impala-sql-py
pyenv activate impala-sql-py
```

### Install Packages

```bash
pip install --upgrade pip setuptools
pip install tree-sitter
pip install ../../tree-sitter-impala
```

### Check Installed Packages

```bash
pip list

Package            Version
------------------ -------
pip                24.1
setuptools         70.1.0
tree-sitter        0.22.3
tree-sitter-impala 0.0.1
```

---

## Run

```bash
python -m impala-sql-py
```

