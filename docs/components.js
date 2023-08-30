$(document).ready(function () {
  $(navbar).replaceAll("#common-navbar");
});

const navbar = `<nav
class="navbar navbar-expand-md navbar-light dark-orange"
>
<div class="container-fluid">
  <a class="navbar-brand" href="#">Jonathan Chen</a>
  <button
    class="navbar-toggler"
    type="button"
    data-bs-toggle="collapse"
    data-bs-target="#navbarNav"
    aria-controls="navbarNav"
    aria-expanded="false"
    aria-label="Toggle navigation"
  >
    <span class="navbar-toggler-icon"></span>
  </button>
  <div class="collapse navbar-collapse" id="navbarNav">
    <ul class="navbar-nav">
      <li class="nav-item">
        <a class="nav-link active" aria-current="page" href="#">Home</a>
      </li>
      <li class="nav-item">
        <a class="nav-link" href="./resume.pdf">Resume (pdf)</a>
      </li>
    </ul>
  </div>
</div>
</nav>
`;
