from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="piston2d",
    version="0.1.1",
    rust_extensions=[RustExtension("piston2d.piston2d", binding=Binding.PyO3)],
    packages=["piston2d"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)