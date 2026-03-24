Aquí tienes la versión definitiva y completa de tu README.md, optimizada para destacar tu perfil como estudiante de ingeniería del TESVB y facilitar la revisión técnica a los jueces mediante Solana Playground.

Copia y pega este contenido en tu archivo:

Micro-Grant TESVB 🚀
Crowdfunding Descentralizado para la Innovación Universitaria
Micro-Grant TESVB es una solución de financiamiento colectivo diseñada para que la comunidad estudiantil del Tecnológico de Estudios Superiores de Valle de Bravo pueda impulsar sus proyectos técnicos y de investigación (como los del Taller de Investigación) de manera transparente, eliminando intermediarios y utilizando la infraestructura de la blockchain de Solana.

💡 El Problema y la Solución
En el entorno académico, muchos proyectos de ingeniería se detienen por falta de capital inicial. Las colectas tradicionales suelen carecer de un registro claro y dependen de la confianza en una persona física.

Micro-Grant TESVB resuelve esto mediante un Escrow (fideicomiso) inteligente:

Seguridad: Los fondos no se depositan en una wallet personal, sino que se bloquean en una PDA (Program Derived Address).

Integridad: El contrato inteligente garantiza que el autor solo pueda retirar los fondos si se alcanza la meta de recaudación establecida.

🛠️ Arquitectura Técnica
El programa está desarrollado con el framework Anchor y utiliza las siguientes innovaciones de Solana:

Bóvedas PDA: Cada proyecto genera su propia dirección de cuenta inmutable basada en semillas: [b"grant", nombre_del_proyecto].

Validación de Autoría: Se utiliza el atributo has_one = autor para asegurar que únicamente el creador del proyecto tenga permisos de retiro.

Manejo de Lamports: Transferencia nativa de SOL entre el donante y la bóveda del programa mediante instrucciones de sistema (system_instruction).

🚀 Guía de Ejecución en Solana Playground (SolPG)
Para facilitar la auditoría del código, este proyecto está diseñado para ejecutarse directamente en el navegador sin instalaciones locales:

1. Preparación en SolPG
Accede a solpg.io.

Crea un nuevo proyecto: selecciona "Anchor (Rust)" y nómbralo micro_grant_tesvb.

Reemplaza el contenido de src/lib.rs con el código proporcionado en este repositorio.

2. Despliegue (Build & Deploy)
Haz clic en el icono de Build (martillo) en la barra lateral.

Tras la compilación exitosa, haz clic en el icono de Deploy (nave espacial).

Asegúrate de estar conectado a la Devnet y tener SOL de prueba (solana airdrop 2 en la terminal).

3. Ejecución del Cliente Técnico
Abre el archivo client/client.ts en el explorador de archivos de SolPG.

Pega el código del cliente incluido en este repositorio.

En la terminal de Playground, ejecuta el comando:

Bash
run
El script realizará automáticamente:

La derivación de las PDAs.

La creación del proyecto en la blockchain.

Una donación de prueba de 0.1 SOL.

La consulta del estado actualizado de la meta.

📊 Mockup del Frontend
Para visualizar la experiencia de usuario final en el TESVB, hemos diseñado un prototipo de interfaz que muestra el avance de las metas "On-chain" y la integración con la wallet de Phantom.

<img width="868" height="557" alt="mockup" src="https://github.com/user-attachments/assets/c94a631e-ce1c-423d-b672-df94ea9f78a3" />


🎓 Créditos
Proyecto desarrollado para el Solana Hackathon LATAM 2026 por:

Estudiante: José Arturo Carbajal Peñaloza, Adán Lopez Bautista, Luis Javier Chavez Buenrrostro Ingeniería en Sistemas Computacionales, 6to Semestre.

Institución: Tecnológico de Estudios Superiores de Valle de Bravo (TESVB).
