<p align="center">
  <a href="https://github.com/hypolia/kanri">
    <img src="https://raw.githubusercontent.com/Hypolia/public-resources/main/kanri.webp" width="200px" alt="Kanri" />
  </a>
</p>

<h3 align="center">Kanri</h3>
<p align="center">The Rust microservice to manage and monitor the game servers</p>

<div align="center">

![][rust-image]
[![license-image]][license-url]

</div>



<br />

**Kanri** (管理) is a microservice written in Rust, designed to manage and monitor the game servers created by the `rikuesuto` service. It acts as a centralised manager, maintaining an overview of the state of the servers in real time.

## ✨ Features

- **Server Registration**: Adds a game server to the list when it is created by `rikuesuto`.
- **Update Status**: Updates server information, such as status or configuration.
- **Deleting Servers**: Deletes servers that are no longer active.
- **REST API**: Enables servers to be accessed and managed via endpoints.
- **Monitoring** *(optional)*: Periodically pings servers to check their availability.


[rust-image]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[license-url]: LICENSE.md
[license-image]: https://img.shields.io/badge/License-Apache_2.0-196f3d?style=for-the-badge&logo=apache&logoColor=white