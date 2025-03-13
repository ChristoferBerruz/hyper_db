FROM postgres:14

# Create necessary directories
RUN mkdir -p /usr/lib/postgresql/14/lib/ /usr/share/postgresql/14/extension/

# Copy the extension files
COPY ./optim/target/release/optim-pg14/usr/lib/postgresql/14/lib/optim.so /usr/lib/postgresql/14/lib/
COPY ./optim/target/release/optim-pg14/usr/share/postgresql/14/extension/optim--0.0.0.sql /usr/share/postgresql/14/extension/
COPY ./optim/target/release/optim-pg14/usr/share/postgresql/14/extension/optim.control /usr/share/postgresql/14/extension/

# Copy and set permissions for the initialization script
COPY init.sh /init.sh
RUN chmod +x /init.sh

# Set the entrypoint to the initialization script
ENTRYPOINT ["/init.sh"]