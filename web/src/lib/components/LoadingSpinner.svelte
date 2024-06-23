<div class="oxygen">
    <div class="orbit outer-orbit">
        <div class="electron one" />
        <div class="electron two" />
        <div class="electron three" />
        <div class="electron four" />
        <div class="electron five" />
        <div class="electron six" />
        <div class="orbit inner-orbit">
            <div class="electron seven" />
            <div class="electron eight" />
            <div class="core" />
        </div>
    </div>
</div>

<style>
    /* Oxygen Spinner */
    /* Heavily adapted from https://codepen.io/AlexWarnes/pen/jXYYKL */

    @keyframes spin {
        from {
            transform: rotate(0);
        }
        to {
            transform: rotate(359deg);
        }
    }

    .oxygen {
        width: 250px;
        height: 250px;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: var(--oxi-color-primary-500);
        border-radius: var(--oxi-rounded-full);
    }

    .orbit {
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
        border: 1px solid #e8edf2;
        border-radius: 50%;
    }

    .outer-orbit {
        width: 165px;
        height: 165px;
        animation: spin 6s linear 0s infinite;
    }

    .inner-orbit {
        width: 90px;
        height: 90px;
        animation: spin 3s linear 0s infinite;
    }

    .electron {
        position: absolute;
        width: 10px;
        height: 10px;
        border-radius: 50%;
        background-color: #e8edf2;
    }

    .electron.one {
        top: -5px;
    }

    .electron.two {
        top: 17px;
        right: 20px;
    }

    .electron.three {
        bottom: 17px;
        right: 20px;
    }

    .electron.four {
        bottom: 17px;
        left: 20px;
    }

    .electron.five {
        top: 17px;
        left: 20px;
    }

    .electron.six {
        bottom: -5px;
    }

    .electron.seven {
        top: -5px;
    }

    .electron.eight {
        bottom: -5px;
    }

    .core {
        width: 35px;
        height: 35px;
        border-radius: 100%;
        background-color: #e8edf2;
    }
</style>
