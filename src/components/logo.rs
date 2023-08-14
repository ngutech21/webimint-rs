use leptos::*;

#[component]
pub fn Logo(cx: Scope) -> impl IntoView {
    // Fedimint Logo - Figma
    // https://www.figma.com/file/ioeAVbWBc3ow2RaZETatM7/Fedimint?type=design&node-id=5520-65113&mode=design&t=T7dxEPEa81LH5HYg-0
    view! { cx,
      <svg width="200" height="46" viewBox="0 0 200 46" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path fill-rule="evenodd" clip-rule="evenodd" d="M26.8363 3.68341C26.8363 5.71771 25.1872 7.36683 23.1529 7.36683C21.1186 7.36683 19.4695 5.71771 19.4695 3.68341C19.4695 1.64912 21.1186 0 23.1529 0C25.1872 0 26.8363 1.64912 26.8363 3.68341ZM42.4491 11.205C42.4491 13.2393 40.8 14.8885 38.7657 14.8885C36.7314 14.8885 35.0822 13.2393 35.0822 11.205C35.0822 9.17074 36.7314 7.52163 38.7657 7.52163C40.8 7.52163 42.4491 9.17074 42.4491 11.205ZM42.6224 31.7826C44.6567 31.7826 46.3058 30.1335 46.3058 28.0992C46.3058 26.0649 44.6567 24.4158 42.6224 24.4158C40.5881 24.4158 38.939 26.0649 38.939 28.0992C38.939 30.1335 40.5881 31.7826 42.6224 31.7826ZM35.5001 41.6474C35.5001 43.6817 33.851 45.3308 31.8167 45.3308C29.7824 45.3308 28.1333 43.6817 28.1333 41.6474C28.1333 39.6131 29.7824 37.964 31.8167 37.964C33.851 37.964 35.5001 39.6131 35.5001 41.6474ZM14.4892 45.3308C16.5234 45.3308 18.1726 43.6817 18.1726 41.6474C18.1726 39.6131 16.5234 37.964 14.4892 37.964C12.4549 37.964 10.8057 39.6131 10.8057 41.6474C10.8057 43.6817 12.4549 45.3308 14.4892 45.3308ZM7.36683 28.0992C7.36683 30.1335 5.71771 31.7826 3.68342 31.7826C1.64912 31.7826 0 30.1335 0 28.0992C0 26.0649 1.64912 24.4158 3.68342 24.4158C5.71771 24.4158 7.36683 26.0649 7.36683 28.0992ZM7.54018 14.8885C9.57447 14.8885 11.2236 13.2393 11.2236 11.205C11.2236 9.17074 9.57447 7.52163 7.54018 7.52163C5.50588 7.52163 3.85676 9.17074 3.85676 11.205C3.85676 13.2393 5.50588 14.8885 7.54018 14.8885ZM29.6066 18.5099L23.1529 15.4022L16.6992 18.5099L15.1051 25.4929L19.5717 31.0923H26.7342L31.2007 25.4929L29.6066 18.5099ZM23.1498 10.9372L23.1529 10.9357H23.1467L23.1498 10.9372ZM13.2077 15.7241L23.1498 10.9372L33.0919 15.7241L35.5496 26.4865L28.6688 35.1162H17.6309L10.7501 26.4865L13.2077 15.7241Z" fill="url(#paint0_radial_5520_65115)"/>
      <path fill-rule="evenodd" clip-rule="evenodd" d="M160.244 14.7103C161.676 14.7103 162.836 13.55 162.836 12.1188C162.836 10.6876 161.676 9.52734 160.244 9.52734C158.813 9.52734 157.653 10.6876 157.653 12.1188C157.653 13.55 158.813 14.7103 160.244 14.7103ZM57.6797 9.90022V35.2966H61.598V24.3399H72.4095V20.7844H61.598V13.4557H73.3528V9.90022H57.6797ZM79.819 34.6798C81.1734 35.4296 82.7577 35.8045 84.5717 35.8045C86.1922 35.8045 87.5225 35.5505 88.5625 35.0426C89.6026 34.5347 90.437 33.9058 91.0659 33.156C91.7189 32.4062 92.2148 31.6927 92.5534 31.0155L89.4695 29.4191C89.0826 30.2415 88.5263 30.9671 87.8006 31.5959C87.0992 32.2248 86.0471 32.5392 84.6443 32.5392C83.1447 32.5392 81.8869 32.0797 80.8711 31.1606C79.8794 30.2173 79.3594 28.9596 79.311 27.3874H93.0613V25.9725C93.0613 24.1585 92.6985 22.5742 91.9729 21.2197C91.2473 19.8653 90.2314 18.8131 88.9253 18.0633C87.6434 17.2893 86.1438 16.9024 84.4266 16.9024C82.6609 16.9024 81.1009 17.2893 79.7464 18.0633C78.4161 18.8131 77.3761 19.8894 76.6263 21.2923C75.8765 22.6709 75.5016 24.2915 75.5016 26.1539V26.5892C75.5016 28.4275 75.8765 30.048 76.6263 31.4508C77.4003 32.8295 78.4645 33.9058 79.819 34.6798ZM89.2519 24.5213H79.3473C79.5408 23.1668 80.0729 22.1146 80.9437 21.3649C81.8386 20.5909 82.9874 20.2039 84.3903 20.2039C85.7931 20.2039 86.9299 20.5909 87.8006 21.3649C88.6714 22.1146 89.1551 23.1668 89.2519 24.5213ZM104.235 35.8045C102.76 35.8045 101.393 35.4417 100.135 34.7161C98.8777 33.9905 97.8619 32.9504 97.0879 31.5959C96.3381 30.2173 95.9632 28.5605 95.9632 26.6255V26.0813C95.9632 24.1705 96.3381 22.5258 97.0879 21.1472C97.8377 19.7685 98.8414 18.7164 100.099 17.9908C101.357 17.2652 102.736 16.9024 104.235 16.9024C105.396 16.9024 106.364 17.0475 107.138 17.3377C107.936 17.628 108.589 18.0029 109.097 18.4624C109.605 18.8978 109.992 19.3573 110.258 19.8411H110.838V9.90022H114.575V35.2966H110.911V32.7569H110.33C109.871 33.5309 109.169 34.2323 108.226 34.8612C107.307 35.49 105.977 35.8045 104.235 35.8045ZM105.287 32.5392C106.908 32.5392 108.238 32.0192 109.278 30.9792C110.342 29.9391 110.874 28.4516 110.874 26.5167V26.1902C110.874 24.2794 110.354 22.804 109.314 21.7639C108.274 20.7239 106.932 20.2039 105.287 20.2039C103.691 20.2039 102.361 20.7239 101.296 21.7639C100.256 22.804 99.7363 24.2794 99.7363 26.1902V26.5167C99.7363 28.4516 100.256 29.9391 101.296 30.9792C102.361 32.0192 103.691 32.5392 105.287 32.5392ZM127.908 17.4103V35.2966H131.644V24.6301C131.644 23.2273 131.995 22.1509 132.697 21.4011C133.398 20.6513 134.329 20.2764 135.49 20.2764C136.579 20.2764 137.413 20.5788 137.993 21.1835C138.598 21.7639 138.9 22.5984 138.9 23.6868V35.2966H142.637V24.6301C142.637 23.2273 142.988 22.1509 143.689 21.4011C144.391 20.6513 145.322 20.2764 146.483 20.2764C147.571 20.2764 148.406 20.5788 148.986 21.1835C149.591 21.7639 149.893 22.5984 149.893 23.6868V35.2966H153.63V23.3966C153.63 22.0179 153.364 20.869 152.832 19.9499C152.3 19.0066 151.587 18.3052 150.692 17.8456C149.797 17.3619 148.769 17.12 147.608 17.12C146.157 17.12 145.032 17.3982 144.234 17.9545C143.46 18.5108 142.867 19.2001 142.456 20.0225H141.875C141.464 19.1759 140.848 18.4866 140.025 17.9545C139.227 17.3982 138.163 17.12 136.832 17.12C135.551 17.12 134.535 17.3619 133.785 17.8456C133.035 18.3294 132.491 18.8857 132.152 19.5145H131.572V17.4103H127.908ZM166.847 35.2966V17.4103H170.511V20.095H171.091C171.43 19.3694 172.035 18.6922 172.905 18.0633C173.776 17.4345 175.07 17.12 176.787 17.12C178.142 17.12 179.339 17.4224 180.379 18.0271C181.443 18.6317 182.278 19.4904 182.883 20.603C183.487 21.6914 183.79 23.0096 183.79 24.5575V35.2966H180.053V24.8478C180.053 23.2998 179.666 22.163 178.892 21.4374C178.118 20.6876 177.053 20.3127 175.699 20.3127C174.151 20.3127 172.905 20.8206 171.962 21.8365C171.043 22.8524 170.583 24.3278 170.583 26.2627V35.2966H166.847ZM191.873 34.317C192.526 34.97 193.397 35.2966 194.485 35.2966H199.419V32.1402H195.682C195.005 32.1402 194.666 31.7773 194.666 31.0517V20.5667H200V17.4103H194.666V11.4966H190.93V17.4103H185.995V20.5667H190.93V31.7048C190.93 32.7932 191.244 33.6639 191.873 34.317ZM158.344 35.2966V17.4103H162.081V35.2966H158.344ZM123.791 12.1188C123.791 13.55 122.631 14.7103 121.2 14.7103C119.769 14.7103 118.608 13.55 118.608 12.1188C118.608 10.6876 119.769 9.52736 121.2 9.52736C122.631 9.52736 123.791 10.6876 123.791 12.1188ZM119.299 35.2966V17.4103H123.036V35.2966H119.299Z" fill="#170022"/>
      <defs>
      <radialGradient id="paint0_radial_5520_65115" cx="0" cy="0" r="1" gradientUnits="userSpaceOnUse" gradientTransform="translate(23.1529 23.8814) rotate(180) scale(25.7909 25.7909)">
      <stop offset="0.507956" stop-color="#4AD6FF"/>
      <stop offset="1" stop-color="#181884"/>
      </radialGradient>
      </defs>
      </svg>

    }
}
