FROM konstin2/maturin

COPY entrypoint.sh /entrypoint.sh

RUN pip install twine

ENTRYPOINT ["/entrypoint.sh"]