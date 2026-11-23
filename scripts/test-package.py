"""Verify every published crate archive includes and passes its test suite."""

import pathlib
import subprocess
import tarfile
import tempfile
import tomllib


def check_member(root, member, version, target):
    manifest = tomllib.loads((root / member / "Cargo.toml").read_text())
    name = manifest["package"]["name"]
    stem = f"{name}-{version}"
    tests = subprocess.check_output(
        ["git", "ls-files", "tests/"], cwd=root / member, text=True
    ).splitlines()
    if not tests:
        raise RuntimeError(f"{name}: no tracked integration tests found")
    subprocess.run(
        ["cargo", "package", "--allow-dirty", "--no-verify", "--package", name,
         "--target-dir", str(target)],
        cwd=root, check=True,
    )
    with tarfile.open(target / "package" / f"{stem}.crate") as archive:
        members = set(archive.getnames())
        missing = [path for path in tests if f"{stem}/{path}" not in members]
        if missing:
            raise RuntimeError(f"{name}: published archive omits tests: {', '.join(missing)}")
        for required in ("LICENSE-MIT", "LICENSE-APACHE", "README.md"):
            if f"{stem}/{required}" not in members:
                raise RuntimeError(f"{name}: published archive omits {required}")
        archive.extractall(target / "unpacked", filter="data")

    # Execute from the extracted archive, with no repository test sources.
    unpacked = target / "unpacked" / stem
    for selection in ["--all-targets", "--doc"]:
        subprocess.run(
            ["cargo", "test", "--locked", selection,
             "--target-dir", str(target / "test-build")],
            cwd=unpacked, check=True,
        )


def main():
    root = pathlib.Path(__file__).resolve().parent.parent
    workspace = tomllib.loads((root / "Cargo.toml").read_text())["workspace"]
    version = workspace["package"]["version"]
    with tempfile.TemporaryDirectory(prefix="store-facade-package-") as directory:
        for member in workspace["members"]:
            check_member(root, member, version, pathlib.Path(directory))


if __name__ == "__main__":
    main()
