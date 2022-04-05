import { Tooltip } from '../../../../../bootstrap-5.1.3'

window.addEventListener('load', () => {
  [].concat(...document.querySelectorAll('[data-bs-toggle="tooltip"]'))
    .map(tooltipNode => new Tooltip(tooltipNode))
})
