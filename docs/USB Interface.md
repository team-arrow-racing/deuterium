# Introduction

To facilitate easy configuration from a computer, a USB interface exists on the battery controller. This USB exposes a pseudo mass-storage class interface for configuration via a `.toml` file as well as a vendor-specific interface for configuration from custom tools.

## Mass-storage

Rather than requiring specialised software to perform basic configuration, we provide a basic configuration interface in the form of a `.toml` file.
