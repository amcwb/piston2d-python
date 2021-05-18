from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="piston2d",
    version="0.1.6",
    rust_extensions=[RustExtension("piston2d.piston2d", binding=Binding.PyO3)],
    packages=["piston2d", "piston2d.window"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)